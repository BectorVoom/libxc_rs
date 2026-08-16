//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta407 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1520;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1521;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1522;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1523;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta407<F: Float>(t14857: F, t2674: F, t243: F, t4423: F, t231: F, t2662: F, t2661: F, t10722: F, t1565: F, t4352: F, t4366: F, t10726: F, t2430: F, t2747: F, t4365: F, t10762: F, t10783: F, t10812: F, t10816: F, t10900: F, t14843: F, t14846: F, t14850: F, t14853: F, t2745: F, t851: F, t10824: F, t10826: F, t10833: F, t10838: F, t10842: F, t10846: F, t10853: F, t10855: F, t10859: F, t10881: F, t10885: F, t10888: F, t10868: F, t241: F, t820: F, t14547: F, t4364: F, t2724: F, t4450: F, t14676: F, t10811: F, t4452: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14859, t14861, t14864, t14866, t14868, t14869) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1520::<F>(t14857, t2674, t243, t4423, t231, t2662, t2661, t10722, t1565, t4352, t4366, t10726);
        let (t14874, t14878) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1521::<F>(t14869, t2661, t231, t2430, t2747, t4365, t10762, t10783, t10812, t10816, t10900, t14843, t14846, t14850, t14853, t14859, t14864, t14866, t2745, t851);
        let t14889 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1522::<F>(t10824, t10826, t10833, t10838, t10842, t10846, t10853, t10855, t10859, t10881, t10885, t10888);
        let (t14894, t14896, t14900, t14904, t14907) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1523::<F>(t10868, t241, t820, t14547, t4364, t4365, t2724, t2747, t4450, t14676, t4366, t10811, t4452);
    (t14861, t14868, t14874, t14878, t14889, t14894, t14896, t14900, t14904, t14907)
}
