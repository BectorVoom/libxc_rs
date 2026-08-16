//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1321/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1321(t28326: f64, t28878: f64, t28881: f64, t28884: f64, t28887: f64, t28889: f64, t28891: f64, t27154: f64, t27327: f64, t27330: f64, t27719: f64, t27725: f64, t27728: f64, t8: f64, t91786: f64, t93848: f64, t93849: f64, t93852: f64, t99743: f64, t99758: f64, t99767: f64, t99786: f64) -> f64 {
    let t99790 = t28326 / 8.0_f64;
    let t99791 = t28878 / 8.0_f64;
    let t99792 = t28881 / 8.0_f64;
    let t99793 = t28884 / 8.0_f64;
    let t99794 = t28887 / 8.0_f64;
    let t99795 = t28889 / 8.0_f64;
    let t99796 = t28891 / 8.0_f64;
    let t99797 = t91786 + t8 * (t99743 + t99758 + t99767 + t99786) - t99790 - t99791 - t27330 - t27725 - t27728 + t27154 - t27327 + t93848 - t27719 - t99792 - t99793 - t93849 - t99794 + t99795 + t99796 + t93852;
    t99797
}
