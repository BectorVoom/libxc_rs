//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 898/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk898<F: Float>(t3119: F, t5110: F, t4336: F, t16236: F, t8537: F, t322: F, t15240: F, t5324: F, t17352: F, t3245: F, t17344: F, t4289: F, t1506: F, t4573: F, t4374: F, t1111: F, t12026: F, t1509: F, t1520: F, t15225: F, t15228: F, t15255: F, t15272: F, t15327: F, t15355: F, t3116: F, t4363: F, t4369: F, t5314: F, t5325: F, t5337: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17662 = t3119 * t5110;
    let t17663 = t4336 * t17662;
    let t17666 = t8537 * t16236;
    let t17667 = t322 * t17666;
    let t17670 = t15240 * t5324;
    let t17674 = t3245 * t17352;
    let t17677 = t4289 * t17344;
    let t17687 = t4573 * t1506;
    let t17688 = t17687 * t3119;
    let t17689 = t4374 * t17688;
    let t17696 = -t15225 / 54.0 + t15228 / 288.0 - 0.1420012659563261767e0 * t3116 * t17663 + t1111 * t17667 / 48.0 + 0.71000632978163088351e-1 * t3116 * t17670 + 0.94667510637550784468e-1 * t15255 - t1111 * t17674 / 48.0 + t1111 * t17677 / 72.0 + 0.35973654042269298099e1 * t15355 * t1509 + 0.18352229811776266582e0 * t15327 * t1520 - 0.91572784804598301689e1 * t15272 - 0.75734008510040627576e0 * t12026 * t5325 + 0.71000632978163088351e-1 * t3116 * t17689 - 0.56800506382530470682e0 * t4363 * t5314 + 0.57954409931925052365e-1 * t4369 * t5337;
    (t17662, t17663, t17666, t17667, t17670, t17674, t17677, t17688, t17689, t17696)
}
