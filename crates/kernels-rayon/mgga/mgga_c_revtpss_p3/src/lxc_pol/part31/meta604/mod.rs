//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2039;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta604(t13730: f64, t2023: f64, t2782: f64, t10073: f64, t25938: f64, t27836: f64, t14079: f64, t26054: f64, t7289: f64, t97925: f64, t2470: f64, t27872: f64, t25895: f64, t1892: f64, t7063: f64, t25877: f64, t25881: f64, t1955: f64, t97960: f64, t213: f64, t27960: f64, t27902: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98001, t98003, t98010, t98011, t98028) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2039(t13730, t2023, t2782, t10073, t25938, t27836, t14079, t26054, t7289, t97925, t2470, t27872);
        let (t98029, t98040, t98041, t98043, t98050, t98056, t98067) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2040(t25895, t98028, t1892, t7063, t25877, t25881, t1955, t97960, t213, t27960, t27902, t686, t72);
    (t98001, t98003, t98010, t98011, t98028, t98029, t98040, t98041, t98043, t98050, t98056, t98067)
}
