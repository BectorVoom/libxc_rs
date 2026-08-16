//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1195/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1195(t27149: f64, t520: f64, t9061: f64, t11449: f64, t11451: f64, t1803: f64, t190: f64, t21183: f64, t11492: f64, t34675: f64, t34772: f64, t34776: f64, t34779: f64, t34782: f64, t34785: f64, t34788: f64, t34791: f64, t34794: f64) -> f64 {
    let t34797 = t9061 * t520 * t27149;
    let t34802 = t1803 * t190 * t11449 * t11451 * t21183;
    let t34804 = t34675 * t11492;
    let t34806 = 0.96684272530105650818e-8_f64 * t34772 - 0.26194992237489957663e-9_f64 * t34776 - 0.6956508833112845217e-4_f64 * t34779 - 0.10020915386217878654e-6_f64 * t34782 - 0.34752370105806885418e-3_f64 * t34785 + 0.2504163411376437654e-5_f64 * t34788 + 0.21720231316129303386e-4_f64 * t34791 + 0.56863432614185654571e-5_f64 * t34794 + 0.14732841528397051554e-8_f64 * t34797 - 0.49166375783284505216e-8_f64 * t34802 + 0.4637672555408563478e-4_f64 * t34804;
    t34806
}
