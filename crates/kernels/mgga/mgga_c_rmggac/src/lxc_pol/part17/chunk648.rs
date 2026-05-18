//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 648/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk648<F: Float>(t5259: F, t8901: F, t4669: F, t8905: F, t2402: F, t321: F, t305: F, t326: F, t8824: F, t8866: F, t8994: F, t8998: F, t9001: F, t9003: F, t9006: F, t9009: F, t9011: F, t9013: F, t9015: F, t9017: F) -> (F, F) {
    let t9021 = t5259 * t8901;
    let t9023 = t4669 * t8905;
    let t9025 = t2402 * t321;
    let t9028 = -F::new(0.59871208509319042821e-1) * t326 * t8824 + F::new(0.59871208509319042821e-1) * t305 * t8994 + F::new(0.39914139006212695213e-1) * t8998 - F::new(0.79828278012425390427e-1) * t9001 + F::new(0.2993560425465952141e-1) * t9003 + F::new(0.2993560425465952141e-1) * t9006 + F::new(0.11974241701863808564e0) * t9009 - F::new(0.8980681276397856423e-1) * t9011 + F::new(0.17961362552795712846e0) * t9013 + F::new(0.44903406381989282115e-1) * t9015 - F::new(0.8980681276397856423e-1) * t9017 - F::new(0.59871208509319042821e-1) * t326 * t8866 - F::new(0.2993560425465952141e-1) * t9021 + F::new(0.44903406381989282115e-1) * t9023 + F::new(0.59871208509319042821e-1) * t305 * t9025;
    (t9025, t9028)
}
