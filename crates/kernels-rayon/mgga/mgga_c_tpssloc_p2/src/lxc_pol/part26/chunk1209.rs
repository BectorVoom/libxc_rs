//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1209/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1209(t22648: f64, t6897: f64, t794: f64, t12021: f64, t12030: f64, t1375: f64, t1386: f64, t3888: f64, t6963: f64, t6992: f64, t80704: f64, t80709: f64, t80711: f64, t80714: f64, t80722: f64, t80725: f64, t80728: f64, t80735: f64) -> f64 {
    let t80738 = t6897 * t794 * t22648;
    let t80740 = -3.0_f64 * t80704 * t1386 - 0.24674011002723396548e-1_f64 * t80709 - 0.78134368175290755733e-1_f64 * t80711 - 0.49348022005446793095e-1_f64 * t80714 + 6.0_f64 * t12030 * t6963 - 18.0_f64 * t1375 * t12021 * t6992 * t3888 + 0.19190897446562641759e0_f64 * t80722 + 0.12337005501361698274e-1_f64 * t80725 - 0.34543615403812755166e0_f64 * t80728 - 0.19739208802178717238e0_f64 * t80735 - 0.12337005501361698274e-1_f64 * t80738;
    t80740
}
