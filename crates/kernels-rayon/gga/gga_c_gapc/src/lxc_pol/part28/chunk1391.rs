//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1391/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1391(t34069: f64, t34071: f64, t34075: f64, t34079: f64, t34084: f64, t34088: f64, t34095: f64, t34098: f64, t34092: f64, t34100: f64, t36812: f64, t34104: f64) -> (f64, f64) {
    let t36813 = 0.40021712703254065176e-7_f64 * t34069;
    let t36814 = 0.40094868252346065012e-6_f64 * t34071;
    let t36815 = 0.26194149710963390811e-8_f64 * t34075;
    let t36816 = 0.32227054270378187512e-8_f64 * t34079;
    let t36817 = 0.60722656250000000004e-3_f64 * t34084;
    let t36818 = 0.88394205998751600035e-8_f64 * t34088;
    let t36820 = 0.67528199161846004232e-6_f64 * t34095;
    let t36821 = 0.78582449132890172432e-8_f64 * t34098;
    let t36823 = -t36812 - t36813 - t36814 - t36815 - t36816 + t36817 + t36818 - 0.98380106748709416168e-8_f64 * t34092 - t36820 + t36821 - 0.3623181683912940217e-6_f64 * t34100;
    let t36824 = 0.4637672555408563478e-4_f64 * t34104;
    (t36823, t36824)
}
