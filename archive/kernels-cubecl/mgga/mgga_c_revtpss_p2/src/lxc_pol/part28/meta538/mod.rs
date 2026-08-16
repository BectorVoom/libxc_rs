//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1983;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1984;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1985;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta538<F: Float>(t1448: F, t1907: F, t28197: F, t28196: F, t7316: F, t7898: F, t13426: F, t1936: F, t18227: F, t4248: F, t7002: F, t27123: F, t4292: F, t93: F, t7889: F, t2322: F, t7741: F, t5523: F, t1312: F, t28042: F, t1518: F, t25805: F, t28025: F, t28030: F, t28160: F, t670: F, t6985: F, t1502: F, t1911: F, t2007: F, t2011: F, t28175: F, t28179: F, t28183: F, t28186: F, t28188: F, t28190: F, t28192: F, t28193: F, t4246: F, t569: F, t5787: F, t7221: F, t7231: F, t27142: F, t28046: F, t28171: F, t3: F, t2042: F, t5795: F, t1916: F, t7331: F, t7334: F, t1459: F, t7950: F, param_d: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28198, t28199, t28201, t28202, t28212, t28214, t28216, t28218) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1983::<F>(t1448, t1907, t28197, t28196, t7316, t7898, t13426, t1936, t18227, t4248, t7002, t27123);
        let (t28219, t28230) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1984::<F>(t4292, t93, t1936, t7002, t7889, t2322, t7741, t5523, t1312, t28042, t1518, t25805, t28025, t28030, t28160, t28212, t28214, t28216, t28218, t670, t6985);
        let t28232 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1985::<F>(t1502, t1911, t2007, t2011, t28175, t28179, t28183, t28186, t28188, t28190, t28192, t28193, t28201, t28202, t28230, t4246, t569, t5787, t7221, t7231);
        let (t28234, t28235, t28246, t28257, t28259, t28261, t28263) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1986::<F>(t27142, t28046, t28171, t28232, t3, t2042, t5795, t1916, t7331, t7334, t1459, t7950, param_d);
    (t28198, t28199, t28219, t28230, t28234, t28235, t28246, t28257, t28259, t28261, t28263)
}
