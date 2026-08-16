//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta256 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk978;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk979;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk980;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk982;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk983;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta256<F: Float>(t1911: F, t2198: F, t1312: F, t2199: F, t2201: F, t4248: F, t651: F, t7732: F, t7889: F, t8393: F, t8407: F, t8411: F, t3: F, param_d: F, t1518: F, t8342: F, t117: F, t8406: F, t1916: F, t1918: F, t2207: F, t2209: F, t572: F, t573: F, t587: F, t65: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F, t121: F, t131: F, t141: F, t22: F, t2456: F, t624: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t8413 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk978::<F>(t1911, t2198);
        let (t8416, t8417) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk979::<F>(t1312, t2199, t2201, t4248, t651, t7732, t7889, t8393, t8407, t8411, t8413, t3);
        let t8421 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk980::<F>(t8416, param_d);
        let (t8427, t8430, t8433, t8779) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk981::<F>(t1518, t8342, t117, t8406, t1916, t1918, t2207, t2209, t572, t573, t8421, t587, t65);
        let (t9275, t9278) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk982::<F>(t143, t2580, t130, t2566, t700, t2584);
        let (t9283, t9285) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk983::<F>(t121, t131, t141, t22, t2456, t624);
    (t8413, t8416, t8417, t8421, t8427, t8430, t8433, t8779, t9275, t9278, t9283, t9285)
}
