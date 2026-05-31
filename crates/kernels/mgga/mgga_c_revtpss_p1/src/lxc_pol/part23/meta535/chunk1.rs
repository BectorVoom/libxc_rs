//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2068/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2068<F: Float>(t10241: F, t5907: F, t661: F, t1509: F, t2: F, t580: F, t2357: F, t5911: F, t21850: F, t108: F, t105: F, t13475: F, t13496: F, t1507: F, t21836: F, t21840: F, t21846: F, t21851: F, t4280: F, t4284: F, t5896: F, t5899: F, t5902: F, t656: F, t662: F, t97: F) -> (F, F, F, F, F, F) {
    let t21860 = t10241 * t5907;
    let t21861 = t21860 * t661;
    let t21864 = t1509 * t2;
    let t21865 = t21864 * t580;
    let t21868 = t2357 * t5911;
    let t21869 = t21868 * t661;
    let t21872 = -t21850;
    let t21873 = t108 * t21872;
    let t21876 = -F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t656 * t5896 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t97 * t21836 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t13475 * t21840 - F::cast_from(25.0_f64) / F::cast_from(9.0_f64) * t656 * t5899 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t97 * t21846 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t97 * t21851 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t5902 * t662 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t1507 * t4280 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1507 * t4284 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t105 * t21861 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t13496 * t21865 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t105 * t21869 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t105 * t21873;
    (t21861, t21865, t21869, t21872, t21873, t21876)
}
