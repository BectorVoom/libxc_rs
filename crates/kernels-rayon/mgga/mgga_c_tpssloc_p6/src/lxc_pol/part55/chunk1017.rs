//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1017/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1017(t11605: f64, t2154: f64, t5059: f64, t225: f64, t8055: f64, t2123: f64, t4930: f64, t1238: f64, t1252: f64, t14972: f64, t15820: f64, t1761: f64, t2121: f64, t2155: f64, t24646: f64, t24893: f64, t27549: f64, t27761: f64, t27767: f64, t27770: f64, t27776: f64, t27779: f64, t27784: f64, t3593: f64, t4945: f64, t5060: f64, t7283: f64, t7351: f64, t7356: f64, t8088: f64) -> (f64, f64, f64, f64) {
    let t27785 = t11605 * t2154;
    let t27786 = t27785 * t5059;
    let t27792 = t8055 * t225;
    let t27794 = t4930 * t2123;
    let t27797 = -t24893 * t1761 + 2.0_f64 * t1238 * t27761 + 0.27415567780803773942e-2_f64 * t24646 + 0.82246703342411321825e-2_f64 * t2121 * t27767 - 0.27415567780803773942e-2_f64 * t27770 + 2.0_f64 * t4945 * t7356 + 0.36554090374405031923e-2_f64 * t27549 * t27776 - 0.27415567780803773942e-2_f64 * t7283 * t27779 + 2.0_f64 * t7351 * t5060 - 6.0_f64 * t27784 * t27786 - t15820 * t2155 - t3593 * t8088 - t14972 * t2155 - t27792 * t1252 - 0.82246703342411321825e-2_f64 * t7283 * t27794;
    (t27785, t27786, t27792, t27797)
}
