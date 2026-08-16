//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1298/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1298(t23159: f64, t23168: f64, t1902: f64, t9971: f64, t226: f64, t23008: f64, t23016: f64, t235: f64, t2617: f64, t6657: f64, t812: f64, t81689: f64, t81691: f64, t81695: f64, t81697: f64, t81702: f64, t81704: f64, t81709: f64, t81713: f64, t81717: f64, t81718: f64, t81976: f64, t81980: f64, t81987: f64, t829: f64, t9661: f64, t9976: f64, t9981: f64) -> f64 {
    let t81989 = t23168 * t23159;
    let t81991 = t9971 * t1902;
    let t82000 = -t81689 + 0.12337005501361698274e-1_f64 * t81691 + 0.14804406601634037928e0_f64 * t81695 + 0.57572692339687925277e-1_f64 * t81697 - 0.24674011002723396548e-1_f64 * t81702 + 0.57572692339687925277e-1_f64 * t81704 - 0.24674011002723396548e-1_f64 * t81709 + 0.49348022005446793095e-1_f64 * t81713 + t81717 - 3.0_f64 * t812 * t81718 * t829 - 3.0_f64 * t2617 * t23016 + t226 * t235 * t81976 - 0.34543615403812755166e0_f64 * t81980 - 0.19739208802178717238e0_f64 * t81987 + 0.11514538467937585055e0_f64 * t81989 - 6.0_f64 * t812 * t81991 * t9976 + 6.0_f64 * t812 * t23008 * t9981 - t812 * t6657 * t9661;
    t82000
}
