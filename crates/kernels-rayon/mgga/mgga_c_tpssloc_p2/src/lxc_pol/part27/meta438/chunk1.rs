//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1766/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1766(t22863: f64, t1995: f64, t9223: f64, t213: f64, t1999: f64, t22805: f64, t22809: f64, t22820: f64, t22826: f64, t22830: f64, t22834: f64, t22837: f64, t22840: f64, t22848: f64, t22850: f64, t22856: f64, t22859: f64, t22861: f64) -> (f64, f64, f64, f64) {
    let t22864 = 35.0_f64 / 432.0_f64 * t22863;
    let t22865 = t9223 * t1995;
    let t22866 = t22865 * t213;
    let t22867 = t22866 * t1999;
    let t22868 = 0.11304371706359309439e-1_f64 * t22867;
    let t22869 = 0.16956557559538964159e-1_f64 * t22805 - 0.12111826828242117256e-2_f64 * t22809 - t22820 + t22826 + 0.24223653656484234512e-2_f64 * t22830 + t22834 / 192.0_f64 + t22837 / 1536.0_f64 + t22840 / 16.0_f64 + 0.84782787797694820792e-2_f64 * t22848 + 5.0_f64 / 384.0_f64 * t22850 + 0.6728792682356731809e-4_f64 * t22856 + t22859 - t22861 + t22864 + t22868;
    (t22864, t22865, t22868, t22869)
}
