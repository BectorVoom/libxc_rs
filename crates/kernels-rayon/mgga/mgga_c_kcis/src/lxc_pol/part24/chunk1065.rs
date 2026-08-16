//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1065/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1065(t27835: f64, t27889: f64, t27939: f64, t27983: f64, t393: f64, t1141: f64, t8060: f64, t1203: f64, t1820: f64, t26868: f64, t26871: f64, t5039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27985 = t27835 + t27889 + t27939 + t27983;
    let t27986 = t27985 * t393;
    let t27987 = t8060 * t1141;
    let t27988 = t27987 * t1203;
    let t27989 = t26868 * t1820;
    let t27991 = 2.0_f64 * t26871 * t5039;
    (t27985, t27986, t27987, t27988, t27989, t27991)
}
