//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 809/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk809(t1235: f64, t477: f64, t1090: f64, t7362: f64, t24837: f64, t3612: f64, t1244: f64, t2121: f64, t24804: f64, t24807: f64, t24812: f64, t24817: f64, t24823: f64, t24827: f64, t24830: f64, t24834: f64, t24838: f64, t24841: f64, t24845: f64, t24849: f64, t24853: f64, t24856: f64, t3610: f64, t3624: f64, t7283: f64, t7373: f64) -> f64 {
    let t24858 = t477 * t1235;
    let t24859 = t24858 * t1090;
    let t24860 = t7362 * t24859;
    let t24863 = t24837 * t3612;
    let t24866 = t1244 * t24804 + 0.82246703342411321825e-2_f64 * t7373 * t24807 + 0.16449340668482264365e-1_f64 * t24812 * t24817 - 0.82246703342411321825e-2_f64 * t24812 * t24823 + 0.54831135561607547884e-2_f64 * t24827 + 0.82246703342411321825e-2_f64 * t2121 * t24830 - 0.16449340668482264365e-1_f64 * t7373 * t24834 - t3624 * t24838 + 2.0_f64 * t1244 * t24841 + 0.54831135561607547884e-2_f64 * t24845 - 0.54831135561607547884e-2_f64 * t24849 * t24853 - 0.18277045187202515961e-2_f64 * t24856 - 0.54831135561607547884e-2_f64 * t7283 * t24860 + 2.0_f64 * t3610 * t24863;
    t24866
}
