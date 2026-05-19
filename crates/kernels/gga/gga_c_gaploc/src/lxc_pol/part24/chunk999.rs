//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 999/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk999<F: Float>(t1022: F, t830: F, t1: F, t787: F, t2631: F, t2628: F, t2976: F, t7284: F, t2639: F, t10627: F, t723: F) -> (F, F, F, F, F, F, F, F) {
    let t10809 = t830 * t1022;
    let t10810 = t10809 * t1;
    let t10811 = t787 * t10810;
    let t10813 = F::cast_from(0.42900587942220512003e1_f64) * t10811 * t2631;
    let t10814 = t2976 * t2628;
    let t10815 = F::cast_from(0.29792074959875355558e-1_f64) * t10814;
    let t10816 = t7284 * t1022;
    let t10817 = t787 * t10816;
    let t10819 = F::cast_from(0.25025342966295298669e1_f64) * t10817 * t2639;
    let t10820 = t10627 * t723;
    (t10809, t10811, t10813, t10815, t10816, t10817, t10819, t10820)
}
