//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1199/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1199<F: Float>(t10824: F, t10826: F, t10833: F, t10838: F, t10842: F, t10846: F, t10853: F, t10855: F, t10859: F, t10881: F, t10885: F, t10888: F) -> F {
    let t14889 = -t10824 + t10826 - F::cast_from(0.12705000702321332056e-4_f64) * t10833 - F::cast_from(0.57165357490759649296e-4_f64) * t10838 - F::cast_from(0.12705000702321332056e-4_f64) * t10842 + F::cast_from(0.27104001498285508387e-3_f64) * t10846 + F::cast_from(0.25410001404642664112e-4_f64) * t10853 + F::cast_from(0.10003937560882938627e-2_f64) * t10855 - F::cast_from(0.20007875121765877254e-2_f64) * t10859 + F::cast_from(0.10003937560882938627e-2_f64) * t10881 - t10885 + F::cast_from(0.2032800112371413129e-4_f64) * t10888;
    t14889
}
