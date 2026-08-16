//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1261/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1261(t112: f64, t27240: f64, t111: f64, t7945: f64, t2022: f64, t671: f64, t7450: f64, t7684: f64, t8944: f64, t1808: f64, t254: f64, t1307: f64, t1842: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94127 = t27240 * t112;
    let t94170 = t7945 * t111;
    let t96351 = t2022 * t671;
    let t96361 = t7450 * t671;
    let t96797 = t7684 * t8944;
    let t97626 = t1808 * t254;
    let t97721 = t1842 * t1307;
    (t94127, t94170, t96351, t96361, t96797, t97626, t97721)
}
