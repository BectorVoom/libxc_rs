//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1522;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1523;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1524;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta405(t14857: f64, t2674: f64, t243: f64, t4423: f64, t231: f64, t2662: f64, t2661: f64, t10722: f64, t1565: f64, t4352: f64, t4366: f64, t10726: f64, t2430: f64, t2747: f64, t4365: f64, t10762: f64, t10783: f64, t10812: f64, t10816: f64, t10900: f64, t14843: f64, t14846: f64, t14850: f64, t14853: f64, t2745: f64, t851: f64, t10824: f64, t10826: f64, t10833: f64, t10838: f64, t10842: f64, t10846: f64, t10853: f64, t10855: f64, t10859: f64, t10881: f64, t10885: f64, t10888: f64, t10868: f64, t241: f64, t820: f64, t14547: f64, t4364: f64, t2724: f64, t4450: f64, t14676: f64, t10811: f64, t4452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14859, t14861, t14864, t14866, t14868, t14869) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1522(t14857, t2674, t243, t4423, t231, t2662, t2661, t10722, t1565, t4352, t4366, t10726);
        let (t14874, t14878) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1523(t14869, t2661, t231, t2430, t2747, t4365, t10762, t10783, t10812, t10816, t10900, t14843, t14846, t14850, t14853, t14859, t14864, t14866, t2745, t851);
        let t14889 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1524(t10824, t10826, t10833, t10838, t10842, t10846, t10853, t10855, t10859, t10881, t10885, t10888);
        let (t14894, t14896, t14900, t14904, t14907) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1525(t10868, t241, t820, t14547, t4364, t4365, t2724, t2747, t4450, t14676, t4366, t10811, t4452);
    (t14861, t14868, t14874, t14878, t14889, t14894, t14896, t14900, t14904, t14907)
}
