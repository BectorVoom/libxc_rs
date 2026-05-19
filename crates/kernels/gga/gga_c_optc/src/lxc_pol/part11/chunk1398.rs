//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1398/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1398<F: Float>(t5392: F, t935: F, t1111: F, t11943: F, t11982: F, t12026: F, t15236: F, t17663: F, t3103: F, t3109: F, t3235: F, t3245: F, t34350: F, t4386: F, t46452: F, t46536: F, t46697: F, t5301: F, t5328: F, t5330: F, t53950: F, t53953: F, t53972: F, t53987: F, t53995: F, t54105: F, t58346: F, t58358: F, t58923: F, t58928: F, t58932: F, t58942: F, t58947: F, t8915: F, t8966: F, t8973: F, t9128: F, t9175: F) -> F {
    let t58956 = t5392 * t935;
    let t58970 = F::cast_from(0.18933502127510156893e0_f64) * t53950 + F::cast_from(0.73258227843678641352e2_f64) * t53953 + F::cast_from(0.61048523203065534458e2_f64) * t8973 * t5301 * t58923 - F::cast_from(0.24419409281226213784e2_f64) * t53972 - F::cast_from(0.30524261601532767229e2_f64) * t8966 * t5301 * t58928 - F::cast_from(0.5680050638253047068e0_f64) * t11982 * t46697 * t58932 + F::new(11.0) / F::new(81.0) * t53987 - F::cast_from(0.37867004255020313788e0_f64) * t53995 + t1111 * t3245 * t58358 / F::new(8.0) + F::cast_from(0.27471835441379490507e2_f64) * t3103 * t58942 * t3109 + F::cast_from(0.1062950724327133642e5_f64) * t11943 * t15236 * t58947 + F::cast_from(0.63777043459628018514e5_f64) * t9175 * t15236 * t8915 * t5328 * t935 - F::cast_from(0.63777043459628018516e5_f64) * t9128 * t15236 * t58956 - F::cast_from(0.14488602482981263091e-1_f64) * t4386 * t3235 * t58346 + F::cast_from(0.18558751053731922476e4_f64) * t46536 * t5330 + F::new(5.0) / F::new(972.0) * t34350 - F::cast_from(0.12234819874517511055e0_f64) * t46452 + F::cast_from(0.3029360340401625103e1_f64) * t12026 * t17663 - F::cast_from(0.20195735602677500687e1_f64) * t54105;
    t58970
}
