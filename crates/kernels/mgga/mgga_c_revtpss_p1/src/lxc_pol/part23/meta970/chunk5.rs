//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3275/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3275<F: Float>(t22074: F, t47248: F, t48712: F, t48792: F, t48794: F, t48797: F, t48814: F, t48827: F, t48829: F, t48833: F, t48848: F, t48849: F, t48851: F, t48853: F, t5627: F, t74425: F, t74427: F, t74429: F, t74437: F, t74461: F, t74469: F) -> F {
    let t86106 = F::cast_from(0.15246000842785598468e-2_f64) * t74425 + F::cast_from(0.30011812682648815881e-2_f64) * t74427 + F::cast_from(0.4065600224742826258e-3_f64) * t74429 - F::cast_from(0.38538502130374707237e-2_f64) * t48792 + F::cast_from(0.34013387707001991332e0_f64) * t48794 - t48797 - F::cast_from(0.54214778996945588152e-4_f64) * t74437 - t48814 + F::cast_from(0.45732285992607719436e-2_f64) * t48827 + F::cast_from(0.33884236873090992593e-6_f64) * t48829 + F::cast_from(0.86700792194318801432e-2_f64) * t48833 - t48848 - F::cast_from(0.15415400852149882895e-1_f64) * t48849 - F::cast_from(0.68026775414003982663e-1_f64) * t48851 + F::cast_from(0.21675198048579700358e-2_f64) * t48853 + F::cast_from(0.30492001685571196935e-3_f64) * t74461 - F::cast_from(0.77173232612525526552e-1_f64) * t48712 * t47248 * t22074 * t5627 + F::cast_from(0.76230004213927992336e-4_f64) * t74469;
    t86106
}
