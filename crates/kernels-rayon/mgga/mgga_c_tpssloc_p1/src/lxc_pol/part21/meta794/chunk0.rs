//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2755/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2755(t40767: f64, t16689: f64, t2655: f64, t46302: f64, t16701: f64, t2427: f64, t13133: f64, t4101: f64, t10126: f64, t16949: f64, t2522: f64, t2523: f64, t39529: f64, t40764: f64, t40766: f64, t40779: f64, t40784: f64, t4314: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58040 = 8.0_f64 * t40767;
    let t58042 = 4.0_f64 * t16689 * t2655;
    let t58046 = 0.2077903092681775651e3_f64 * t46302;
    let t58047 = t2427 * t16701;
    let t58048 = 8.0_f64 * t58047;
    let t58052 = t13133 * t4101;
    let t58053 = 16.0_f64 * t58052;
    let t58054 = 3.0_f64 * t10126 * t2522 * t5544 + 12.0_f64 * t16949 * t2523 * t4314 - t39529 + t40764 + t40766 - t40779 + t40784 + t58040 + t58042 + t58046 + t58048 + t58053;
    (t58040, t58042, t58046, t58048, t58053, t58054)
}
