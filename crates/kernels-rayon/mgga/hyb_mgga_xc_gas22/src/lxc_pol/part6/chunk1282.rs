//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1282/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1282(t1230: f64, t7867: f64, t10083: f64, t1867: f64, t23577: f64, t23588: f64, t23591: f64, t23625: f64, t23667: f64, t27649: f64, t27719: f64, t27721: f64, t27723: f64, t27725: f64, t27728: f64, t27732: f64, t27741: f64, t2970: f64, t2971: f64, t2972: f64, t2974: f64, t2987: f64, t3: f64, t3112: f64, t3919: f64, t547: f64, t555: f64, t7842: f64, t7843: f64, t7857: f64, t7861: f64, t7866: f64, t7868: f64, t7920: f64, t9829: f64) -> f64 {
    let t27753 = t7867 * t1230;
    let t27757 = -t23577 / 8.0_f64 - t555 * t2987 * t7920 * t3 / 8.0_f64 - t23588 / 96.0_f64 - t23591 / 24.0_f64 - 41.0_f64 / 144.0_f64 * t23625 - 3.0_f64 / 64.0_f64 * t1867 * t3919 - 3.0_f64 / 32.0_f64 * t547 * t10083 + t27719 / 96.0_f64 - t27721 / 32.0_f64 - t27723 / 32.0_f64 - t27725 / 32.0_f64 + t7842 * t2972 * t27728 / 8.0_f64 + t7842 * t2972 * t27732 / 16.0_f64 + 7.0_f64 / 18.0_f64 * t23667 * t7868 * t27649 - t27741 / 36.0_f64 - t2970 * t2971 * t3112 * t2974 / 12.0_f64 - t2970 * t9829 * t7857 / 12.0_f64 - t2970 * t9829 * t7861 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t7866 * t27753 * t7843;
    t27757
}
