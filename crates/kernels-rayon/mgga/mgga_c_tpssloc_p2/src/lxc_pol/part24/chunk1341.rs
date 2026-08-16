//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1341/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1341(t1906: f64, t82045: f64, t23035: f64, t23153: f64, t2379: f64, t6637: f64, t22984: f64, t22992: f64, t2617: f64, t2679: f64, t2684: f64, t4291: f64, t6657: f64, t6658: f64, t812: f64, t82003: f64, t82005: f64, t82011: f64, t82013: f64, t82016: f64, t82021: f64, t82025: f64, t82028: f64, t82032: f64, t82034: f64, t82039: f64, t82043: f64, t829: f64, t9612: f64, t9958: f64) -> f64 {
    let t82046 = t82045 * t1906;
    let t82047 = 0.27720185200590482541e0_f64 * t82046;
    let t82050 = t23035 * t6637 * t23153 * t2379;
    let t82060 = -0.82246703342411321825e-2_f64 * t82003 + 0.11514538467937585055e0_f64 * t82005 - t812 * t6657 * t9958 - 3.0_f64 * t2617 * t22984 - 0.19190897446562641759e0_f64 * t82011 - 0.11514538467937585055e0_f64 * t82013 - 0.24674011002723396548e-1_f64 * t82016 - 0.49348022005446793095e-1_f64 * t82021 + 0.49348022005446793095e-1_f64 * t82025 + 0.12337005501361698274e-1_f64 * t82028 - 0.78134368175290755733e-1_f64 * t82032 - 3.0_f64 * t4291 * t82034 * t829 - 0.15626873635058151147e0_f64 * t82039 + 0.82246703342411321825e-2_f64 * t82043 - t82047 + 0.14804406601634037928e0_f64 * t82050 - 3.0_f64 * t812 * t22992 * t2684 - 3.0_f64 * t812 * t22992 * t2679 - 3.0_f64 * t9612 * t6658;
    t82060
}
