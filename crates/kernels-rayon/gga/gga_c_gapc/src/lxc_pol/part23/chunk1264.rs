//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1264/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1264(t3635: f64, t8521: f64, t11198: f64, t1928: f64, t2903: f64, t11199: f64, t8422: f64, t11223: f64, t11257: f64, t1577: f64, t35588: f64, t35591: f64, t35595: f64, t35597: f64, t35599: f64, t35601: f64, t35606: f64, t35610: f64, t35613: f64) -> f64 {
    let t35615 = t8521 * t3635;
    let t35618 = t2903 * t11198 * t1928;
    let t35620 = t8422 * t11199;
    let t35623 = t11257 * t11223 * t1577;
    let t35625 = -0.60736713313768998074e-4_f64 * t35588 + 0.12147342662753799615e-3_f64 * t35591 - 0.3218855744218122075e-7_f64 * t35595 + 0.86898242813537603824e-5_f64 * t35597 + 0.52638484871933131664e-3_f64 * t35599 + 0.52638484871933131664e-3_f64 * t35601 - 0.91570008211517136795e-6_f64 * t35606 + 0.1545050757224698596e-4_f64 * t35610 + 0.1545050757224698596e-4_f64 * t35613 + 0.60736713313768998074e-4_f64 * t35615 + 0.86898242813537603824e-5_f64 * t35618 - 0.40552513312984215118e-4_f64 * t35620 + 0.86898242813537603824e-4_f64 * t35623;
    t35625
}
