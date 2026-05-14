//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1318/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1318<F: Float>(t105884: F, t106144: F, t106160: F, t120086: F, t120090: F, t120093: F, t120096: F, t120099: F, t120103: F, t95330: F, t96140: F, t96143: F, t120891: F, t120903: F, t120915: F, t120928: F, t120942: F, t120951: F, t120962: F, t120972: F, t120987: F, t120998: F, t121009: F, t121016: F, t121028: F, t121040: F, t121051: F) -> (F,) {
    let t121060 = t120086 / 12.0 - t120090 / 3.0 - 4.0 / 27.0 * t105884 - t106144 - 2.0 / 27.0 * t95330 + t96140 + t96143 - t106160 + 2.0 / 3.0 * t120093 + 4.0 / 9.0 * t120096 - 4.0 / 27.0 * t120099 - t120103 / 9.0;
    let t121064 = t120891 + t120903 + t120915 + t120928 + t120942 + t120951 + t120962 + t120972 + t120987 + t120998 + t121009 + t121016 + t121028 + t121040 + t121051 + t121060;
    (t121064,)
}
