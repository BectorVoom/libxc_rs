//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1205/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1205<F: Float>(t18386: F, t6154: F, t51892: F, t6930: F, t1091: F, t110543: F, t110641: F, t110659: F, t110950: F, t11593: F, t13885: F, t18498: F, t18622: F, t18694: F, t1901: F, t242: F, t24599: F, t24793: F, t2599: F, t2606: F, t28404: F, t31118: F, t3870: F, t446: F, t4973: F, t51853: F, t6161: F, t6162: F, t68626: F, t97740: F, t97770: F, t97772: F) -> (F, F, F) {
    let t122552 = t6154 * t18386;
    let t122587 = t51892 * t6930;
    let t122591 = -t446 * t242 * t122552 / 3.0 + 4.0 / 81.0 * t97740 + 2.0 / 9.0 * t1901 * t110950 * t3870 + 4.0 / 9.0 * t1901 * t28404 * t18694 - 8.0 / 9.0 * t11593 * t24793 * t18498 - 4.0 / 27.0 * t110641 + 2.0 / 9.0 * t1901 * t2599 * t110543 * t1091 + 4.0 / 27.0 * t97770 + 8.0 / 27.0 * t97772 + t1901 * t68626 * t6162 / 9.0 - 4.0 / 3.0 * t1901 * t13885 * t6161 * t18622 - 4.0 / 3.0 * t1901 * t51853 * t31118 + t110659 + t1901 * t2606 * t24599 * t4973 / 9.0 + 4.0 / 3.0 * t446 * t242 * t122587;
    (t122552, t122587, t122591)
}
