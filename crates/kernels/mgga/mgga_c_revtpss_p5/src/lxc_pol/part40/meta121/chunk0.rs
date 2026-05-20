//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 605/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk605<F: Float>(t225: F, t2760: F, t213: F, t860: F, t256: F, t866: F, t886: F, t2435: F, t871: F, t785: F) -> (F, F, F, F, F, F, F, F) {
    let t2761 = t2760 * t225;
    let t2765 = t213 * t860;
    let t2769 = F::new(1.0) / t866 / t256;
    let t2770 = t225 * t2769;
    let t2771 = t886 * t886;
    let t2772 = t2770 * t2771;
    let t2776 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t871;
    let t2777 = t785 * t225;
    (t2761, t2765, t2769, t2770, t2771, t2772, t2776, t2777)
}
