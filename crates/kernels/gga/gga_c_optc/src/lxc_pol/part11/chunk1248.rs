//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1248/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1248<F: Float>(t1442: F, t52330: F, t52331: F, t19: F, t5328: F, t8974: F, t4356: F, t3119: F, t55127: F, t5311: F, t3104: F, t438: F, t935: F, t5392: F, t1111: F, t11943: F, t11982: F, t12026: F, t15236: F, t17663: F, t3103: F, t3109: F, t3235: F, t3245: F, t34350: F, t4386: F, t46452: F, t46536: F, t46697: F, t5301: F, t5330: F, t53950: F, t53953: F, t53972: F, t53987: F, t53995: F, t54105: F, t58346: F, t58358: F, t8915: F, t8966: F, t8973: F, t9128: F, t9175: F) -> (F, F, F, F, F, F, F) {
    let t58917 = t52330 * t52331 * t1442;
    let t58922 = t5328 * t19;
    let t58923 = t58922 * t8974;
    let t58928 = t58922 * t4356;
    let t58932 = t55127 * t3119;
    let t58941 = t5311 * t5311;
    let t58942 = t3104 * t58941;
    let t58947 = t5328 * t935 * t438;
    let t58956 = t5392 * t935;
    let t58970 = 0.18933502127510156893e0 * t53950 + 0.73258227843678641352e2 * t53953 + 0.61048523203065534458e2 * t8973 * t5301 * t58923 - 0.24419409281226213784e2 * t53972 - 0.30524261601532767229e2 * t8966 * t5301 * t58928 - 0.5680050638253047068e0 * t11982 * t46697 * t58932 + 11.0 / 81.0 * t53987 - 0.37867004255020313788e0 * t53995 + t1111 * t3245 * t58358 / 8.0 + 0.27471835441379490507e2 * t3103 * t58942 * t3109 + 0.1062950724327133642e5 * t11943 * t15236 * t58947 + 0.63777043459628018514e5 * t9175 * t15236 * t8915 * t5328 * t935 - 0.63777043459628018516e5 * t9128 * t15236 * t58956 - 0.14488602482981263091e-1 * t4386 * t3235 * t58346 + 0.18558751053731922476e4 * t46536 * t5330 + 5.0 / 972.0 * t34350 - 0.12234819874517511055e0 * t46452 + 0.3029360340401625103e1 * t12026 * t17663 - 0.20195735602677500687e1 * t54105;
    (t58917, t58923, t58928, t58932, t58941, t58942, t58970)
}
