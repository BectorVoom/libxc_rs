//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2039;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2040;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta604<F: Float>(t13730: F, t2023: F, t2782: F, t10073: F, t25938: F, t27836: F, t14079: F, t26054: F, t7289: F, t97925: F, t2470: F, t27872: F, t25895: F, t1892: F, t7063: F, t25877: F, t25881: F, t1955: F, t97960: F, t213: F, t27960: F, t27902: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t98001, t98003, t98010, t98011, t98028) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2039::<F>(t13730, t2023, t2782, t10073, t25938, t27836, t14079, t26054, t7289, t97925, t2470, t27872);
        let (t98029, t98040, t98041, t98043, t98050, t98056, t98067) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2040::<F>(t25895, t98028, t1892, t7063, t25877, t25881, t1955, t97960, t213, t27960, t27902, t686, t72);
    (t98001, t98003, t98010, t98011, t98028, t98029, t98040, t98041, t98043, t98050, t98056, t98067)
}
