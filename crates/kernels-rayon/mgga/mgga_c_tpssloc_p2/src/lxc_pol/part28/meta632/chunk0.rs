//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1989/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1989(t87709: f64, t87714: f64, t87718: f64, t87729: f64, t87733: f64, t13453: f64, t2613: f64, t26656: f64, t26657: f64, t2679: f64, t4162: f64, t4182: f64, t4281: f64, t4291: f64, t7104: f64, t7839: f64, t82028: f64, t82032: f64, t82039: f64, t85027: f64, t87692: f64, t87699: f64, t87705: f64, t87726: f64, t92552: f64) -> f64 {
    let t92810 = 0.76763589786250567036e-1_f64 * t87709;
    let t92811 = 0.9869604401089358619e-1_f64 * t87714;
    let t92817 = 0.10417915756705434098e0_f64 * t87718;
    let t92822 = 0.16449340668482264365e-1_f64 * t87729;
    let t92825 = 0.76763589786250567036e-1_f64 * t87733;
    let t92826 = -0.16449340668482264365e-1_f64 * t87692 + 0.82246703342411321825e-2_f64 * t82028 + 0.19739208802178717238e0_f64 * t87699 + 0.6579736267392905746e-1_f64 * t87705 - 0.10417915756705434098e0_f64 * t82032 - 0.20835831513410868196e0_f64 * t82039 + t92810 - t92811 + 4.0_f64 * t4281 * t92552 * t4182 + 4.0_f64 * t13453 * t26657 - t85027 - t92817 + t2613 * t7839 + 2.0_f64 * t4162 * t7104 - 0.3289868133696452873e-1_f64 * t87726 + t92822 - t4291 * t26656 * t2679 - t92825;
    t92826
}
