//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1028/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1028(t77840: f64, t14516: f64, t8526: f64, t2329: f64, t71882: f64, t76148: f64, t71852: f64, t71854: f64, t76141: f64, t77830: f64, t77833: f64, t77835: f64, t77836: f64, t77837: f64, t77839: f64) -> f64 {
    let t77841 = 0.10227998120342003148e-1_f64 * t77840;
    let t77842 = t14516 * t8526;
    let t77843 = 0.10227998120342003148e-1_f64 * t77842;
    let t77844 = t71882 * t2329;
    let t77845 = 0.13637330827122670864e-1_f64 * t77844;
    let t77846 = 0.40911992481368012596e-1_f64 * t76148;
    let t77847 = -t71852 - t76141 + t71854 - t77830 - t77833 - t77835 - t77836 - t77837 - t77839 + t77841 + t77843 - t77845 + t77846;
    t77847
}
