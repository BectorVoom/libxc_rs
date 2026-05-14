//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1176/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1176<F: Float>(t34066: F, t34069: F, t34071: F, t34075: F, t34079: F, t34084: F, t34088: F, t34095: F, t34098: F, t34104: F, t34108: F, t34111: F, t34114: F, t34117: F, t34119: F, t34121: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36812 = 0.67528199161846004232e-6 * t34066;
    let t36813 = 0.40021712703254065176e-7 * t34069;
    let t36814 = 0.40094868252346065012e-6 * t34071;
    let t36815 = 0.26194149710963390811e-8 * t34075;
    let t36816 = 0.32227054270378187512e-8 * t34079;
    let t36817 = 0.60722656250000000004e-3 * t34084;
    let t36818 = 0.88394205998751600035e-8 * t34088;
    let t36820 = 0.67528199161846004232e-6 * t34095;
    let t36821 = 0.78582449132890172432e-8 * t34098;
    let t36824 = 0.4637672555408563478e-4 * t34104;
    let t36825 = 0.11272120794395814009e-6 * t34108;
    let t36826 = 0.69504740211613770836e-3 * t34111;
    let t36827 = 0.49163213094075520836e-7 * t34114;
    let t36828 = 0.24581606547037760418e-8 * t34117;
    let t36829 = 0.70341874126922921074e-8 * t34119;
    let t36830 = 0.70341874126922921074e-8 * t34121;
    (t36812, t36813, t36814, t36815, t36816, t36817, t36818, t36820, t36821, t36824, t36825, t36826, t36827, t36828, t36829, t36830)
}
