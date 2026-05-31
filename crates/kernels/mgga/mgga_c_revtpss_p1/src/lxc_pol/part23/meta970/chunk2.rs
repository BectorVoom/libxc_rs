//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3272/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3272<F: Float>(t1395: F, t1879: F, t22223: F, t22229: F, t22237: F, t22240: F, t22246: F, t225: F, t22936: F, t541: F, t543: F, t5644: F, t5652: F, t5655: F, t6832: F, t6837: F, t6840: F, t73: F, t85892: F, t85901: F, t85907: F, t85915: F, t85927: F, t85977: F, t85988: F, t85995: F, t86052: F) -> F {
    let t86054 = (-(t85892 + t85901 + t85907 + t85915 + t85927 + t85977 + t85988 + t85995) * t225 * t541 + F::cast_from(3.0_f64) * t22936 * t1395 + F::cast_from(9.0_f64) * t22223 * t1879 - F::cast_from(36.0_f64) * t6832 * t73 * t5652 + F::cast_from(9.0_f64) * t6832 * t5655 - F::cast_from(36.0_f64) * t5644 * t6837 + F::cast_from(180.0_f64) * t22229 * t22237 - F::cast_from(72.0_f64) * t22229 * t22240 + F::cast_from(9.0_f64) * t5644 * t6840 - F::cast_from(36.0_f64) * t22229 * t22246 + t86052) * t543;
    t86054
}
