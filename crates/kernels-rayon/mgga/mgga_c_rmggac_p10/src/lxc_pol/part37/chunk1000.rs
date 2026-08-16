//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1000/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1000(t78068: f64, t14444: f64, t8377: f64, t27048: f64, t76337: f64, t76340: f64, t76343: f64, t69404: f64, t570: f64, t71916: f64, t8940: f64, t71983: f64, t8626: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78069 = 0.68186654135613354322e-2_f64 * t78068;
    let t78070 = t14444 * t8377;
    let t78072 = 0.35922725105591425692e0_f64 * t27048 * t78070;
    let t78073 = 0.14967802127329760705e-1_f64 * t76337;
    let t78077 = 0.13637330827122670865e0_f64 * t76340;
    let t78078 = 0.5454932330849068346e-1_f64 * t76343;
    let t78079 = 0.79828278012425390427e-1_f64 * t69404;
    let t78083 = 0.11974241701863808564e0_f64 * t8940 * t71916 * t570;
    let t78090 = t71983 * t8626;
    (t78069, t78070, t78072, t78073, t78077, t78078, t78079, t78083, t78090)
}
