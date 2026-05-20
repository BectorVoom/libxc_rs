//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1417/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1417<F: Float>(t10241: F, t5907: F, t661: F, t1509: F, t2: F, t580: F, t2357: F, t5911: F, t21850: F, t108: F, t105: F, t13475: F, t13496: F, t1507: F, t21836: F, t21840: F, t21846: F, t21851: F, t4280: F, t4284: F, t5896: F, t5899: F, t5902: F, t656: F, t662: F, t97: F) -> F {
    let t21860 = t10241 * t5907;
    let t21861 = t21860 * t661;
    let t21864 = t1509 * t2;
    let t21865 = t21864 * t580;
    let t21868 = t2357 * t5911;
    let t21869 = t21868 * t661;
    let t21872 = -t21850;
    let t21873 = t108 * t21872;
    let t21876 = -F::new(50.0) / F::new(27.0) * t656 * t5896 - F::new(10.0) / F::new(27.0) * t97 * t21836 + F::new(20.0) / F::new(9.0) * t13475 * t21840 - F::new(25.0) / F::new(9.0) * t656 * t5899 + F::new(10.0) / F::new(9.0) * t97 * t21846 + F::new(5.0) / F::new(3.0) * t97 * t21851 + F::new(200.0) / F::new(27.0) * t5902 * t662 - F::new(100.0) / F::new(27.0) * t1507 * t4280 + F::new(50.0) / F::new(9.0) * t1507 * t4284 - F::new(10.0) / F::new(27.0) * t105 * t21861 - F::new(20.0) / F::new(9.0) * t13496 * t21865 + F::new(10.0) / F::new(9.0) * t105 * t21869 + F::new(5.0) / F::new(3.0) * t105 * t21873;
    t21876
}
