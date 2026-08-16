//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1983;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1984;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1985;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta538(t1448: f64, t1907: f64, t28197: f64, t28196: f64, t7316: f64, t7898: f64, t13426: f64, t1936: f64, t18227: f64, t4248: f64, t7002: f64, t27123: f64, t4292: f64, t93: f64, t7889: f64, t2322: f64, t7741: f64, t5523: f64, t1312: f64, t28042: f64, t1518: f64, t25805: f64, t28025: f64, t28030: f64, t28160: f64, t670: f64, t6985: f64, t1502: f64, t1911: f64, t2007: f64, t2011: f64, t28175: f64, t28179: f64, t28183: f64, t28186: f64, t28188: f64, t28190: f64, t28192: f64, t28193: f64, t4246: f64, t569: f64, t5787: f64, t7221: f64, t7231: f64, t27142: f64, t28046: f64, t28171: f64, t3: f64, t2042: f64, t5795: f64, t1916: f64, t7331: f64, t7334: f64, t1459: f64, t7950: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28198, t28199, t28201, t28202, t28212, t28214, t28216, t28218) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1983(t1448, t1907, t28197, t28196, t7316, t7898, t13426, t1936, t18227, t4248, t7002, t27123);
        let (t28219, t28230) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1984(t4292, t93, t1936, t7002, t7889, t2322, t7741, t5523, t1312, t28042, t1518, t25805, t28025, t28030, t28160, t28212, t28214, t28216, t28218, t670, t6985);
        let t28232 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1985(t1502, t1911, t2007, t2011, t28175, t28179, t28183, t28186, t28188, t28190, t28192, t28193, t28201, t28202, t28230, t4246, t569, t5787, t7221, t7231);
        let (t28234, t28235, t28246, t28257, t28259, t28261, t28263) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1986(t27142, t28046, t28171, t28232, t3, t2042, t5795, t1916, t7331, t7334, t1459, t7950, param_d);
    (t28198, t28199, t28219, t28230, t28234, t28235, t28246, t28257, t28259, t28261, t28263)
}
