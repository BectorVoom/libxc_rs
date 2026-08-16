//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 567/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk567(t3307: f64, t9420: f64, t813: f64, t3209: f64, t5750: f64, t723: f64, t1445: f64, t9595: f64, t9730: f64, t9953: f64, t9603: f64, t3280: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9981 = t9420 * t3307;
    let t9982 = t813 * t9981;
    let t9984 = t5750 * t3209;
    let t9985 = t9984 * t723;
    let t9986 = t1445 * t9985;
    let t9989 = t9595 * t723;
    let t9990 = t1445 * t9989;
    let t9993 = t1445 * t9730;
    let t9996 = t9953 * t723;
    let t9997 = t1445 * t9996;
    let t10000 = t9603 * t723;
    let t10001 = t1445 * t10000;
    let t10004 = t549 * t3280;
    (t9982, t9986, t9989, t9990, t9993, t9997, t10001, t10004)
}
