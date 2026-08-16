//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1388/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1388<F: Float>(t34069: F, t34071: F, t34075: F, t34079: F, t34084: F, t34088: F, t34095: F, t34098: F, t34092: F, t34100: F, t36812: F, t34104: F) -> (F, F) {
    let t36813 = F::cast_from(0.40021712703254065176e-7_f64) * t34069;
    let t36814 = F::cast_from(0.40094868252346065012e-6_f64) * t34071;
    let t36815 = F::cast_from(0.26194149710963390811e-8_f64) * t34075;
    let t36816 = F::cast_from(0.32227054270378187512e-8_f64) * t34079;
    let t36817 = F::cast_from(0.60722656250000000004e-3_f64) * t34084;
    let t36818 = F::cast_from(0.88394205998751600035e-8_f64) * t34088;
    let t36820 = F::cast_from(0.67528199161846004232e-6_f64) * t34095;
    let t36821 = F::cast_from(0.78582449132890172432e-8_f64) * t34098;
    let t36823 = -t36812 - t36813 - t36814 - t36815 - t36816 + t36817 + t36818 - F::cast_from(0.98380106748709416168e-8_f64) * t34092 - t36820 + t36821 - F::cast_from(0.3623181683912940217e-6_f64) * t34100;
    let t36824 = F::cast_from(0.4637672555408563478e-4_f64) * t34104;
    (t36823, t36824)
}
