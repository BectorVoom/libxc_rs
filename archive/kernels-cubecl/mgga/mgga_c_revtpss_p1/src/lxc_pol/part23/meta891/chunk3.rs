//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2844/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2844<F: Float>(t23148: F, t236: F, t807: F, t854: F, t1559: F, t18599: F, t2661: F, t2662: F, t40862: F, t51099: F, t51100: F, t51102: F, t51104: F, t51122: F, t51170: F, t62216: F, t62236: F, t62241: F, t62246: F, t62251: F, t62392: F, t62399: F, t62401: F, t62405: F) -> F {
    let t76878 = t807 * t236 * t854 * t23148;
    let t76882 = t2661 * t2662 * t18599 * t1559;
    let t76884 = F::cast_from(0.15246000842785598467e-3_f64) * t62216 - t51099 - F::cast_from(0.38538502130374707237e-2_f64) * t51100 + F::cast_from(0.91464571985215438872e-3_f64) * t51102 + F::cast_from(0.11337795902333997111e0_f64) * t51104 - F::cast_from(0.76230004213927992336e-4_f64) * t62236 - F::cast_from(0.38115002106963996168e-4_f64) * t62241 + t51122 + F::cast_from(0.34299214494455789578e-3_f64) * t62246 - F::cast_from(0.54214778996945588152e-4_f64) * t62251 + F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t40862 - F::cast_from(0.38115002106963996168e-4_f64) * t62392 - F::cast_from(0.17006693853500995666e-1_f64) * t62399 + F::cast_from(0.34013387707001991332e-1_f64) * t62401 + F::cast_from(0.7623000421392799234e-3_f64) * t62405 + F::cast_from(0.86700792194318801432e-2_f64) * t51170 + F::cast_from(0.28582678745379824648e-4_f64) * t76878 + F::cast_from(0.42874018118069736973e-3_f64) * t76882;
    t76884
}
