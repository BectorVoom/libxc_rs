//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3285/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3285<F: Float>(t14100: F, t22399: F, t1904: F, t213: F, t22390: F, t225: F, t47504: F, t47512: F, t47886: F, t47899: F, t47904: F, t561: F, t5728: F, t73666: F, t73671: F, t73673: F, t73676: F, t73705: F, t73707: F, t74802: F, t85509: F, t86280: F) -> F {
    let t86285 = t14100 * t22399;
    let t86291 = -F::cast_from(0.7805952431506226415e-1_f64) * t73666 + F::cast_from(0.98781737744032673976e-1_f64) * t73671 + t47504 - F::cast_from(0.19756347548806534796e1_f64) * t74802 * t1904 - F::cast_from(0.21951497276451705328e-1_f64) * t73673 - F::cast_from(0.65854491829355115984e-1_f64) * t73676 + F::cast_from(0.39512695097613069592e1_f64) * t22390 * t5728 - F::cast_from(0.54878743191129263322e-2_f64) * t85509 - F::cast_from(0.11044544084478153697e-3_f64) * t47512 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t86280 * t225 * t561 - F::cast_from(0.29272321618148349057e-1_f64) * t86285 - t47886 - F::cast_from(0.39029762157531132076e-2_f64) * t47899 - F::cast_from(0.91069445034239308177e-1_f64) * t47904 + F::cast_from(0.16463622957338778996e-1_f64) * t73705 + F::cast_from(0.43902994552903410656e-1_f64) * t73707;
    t86291
}
