//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1720;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1721;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta498(t26656: f64, t4182: f64, t7823: f64, t814: f64, t829: f64, t25293: f64, t25317: f64, t226: f64, t23187: f64, t25274: f64, t25285: f64, t25289: f64, t25301: f64, t25304: f64, t25308: f64, t25310: f64, t25314: f64, t25322: f64, t25326: f64, t26613: f64, t26654: f64, t4281: f64, t4291: f64, t7839: f64, t808: f64, t812: f64, t26611: f64, t858: f64, t25036: f64, t25042: f64, t25047: f64, t25056: f64, t25061: f64, t2597: f64, t26582: f64, t26591: f64, t2713: f64, t4147: f64, t4268: f64, t4273: f64, t7087: f64, t7092: f64, t7107: f64, t7830: f64, t855: f64, t2053: f64, t2718: f64, t4300: f64, t13463: f64, t1528: f64, t2054: f64, t23207: f64, t23209: f64, t23233: f64, t23236: f64, t24291: f64, t24305: f64, t25194: f64, t4301: f64, t7842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26657, t26661, t26662, t26676, t26678) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1720(t26656, t4182, t7823, t814, t829, t25293, t25317, t226, t23187, t25274, t25285, t25289, t25301, t25304, t25308, t25310, t25314, t25322, t25326, t26613, t26654, t4281, t4291, t7839, t808, t812);
        let (t26679, t26680, t26684) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1721(t26611, t26678, t858, t25036, t25042, t25047, t25056, t25061, t2597, t26582, t26591, t2713, t4147, t4268, t4273, t7087, t7092, t7107, t7830, t855);
        let (t26690, t26698) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1722(t2053, t2718, t4300, t13463, t1528, t2054, t23207, t23209, t23233, t23236, t24291, t24305, t25194, t2713, t4147, t4268, t4301, t7087, t7092, t7107, t7842, t855);
    (t26657, t26661, t26662, t26676, t26679, t26680, t26684, t26690, t26698)
}
