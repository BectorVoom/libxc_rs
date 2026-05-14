//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1232/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1232<F: Float>(t5820: F, t77143: F, t1290: F, t30062: F, t554: F, t104682: F, t105212: F, t118714: F, t118723: F, t118726: F, t16825: F, t16887: F, t1737: F, t22767: F, t23705: F, t23774: F, t26604: F, t30058: F, t30071: F, t30072: F, t5570: F, t5579: F, t5593: F, t5813: F, t5829: F, t72: F) -> (F, F) {
    let t118729 = t77143 * t5820;
    let t118744 = t1290 * t30062 * t554;
    let t118750 = 0.4445200072839506173e-1 * t23705 * t5570 * t1737 * t118714 + 0.80013601311111111112e0 * t23774 * t22767 * t30058 - 0.10001700163888888889e0 * t118723 + 0.24167761770734866964e0 * t118726 * t5593 - 0.24167761770734866964e0 * t118729 * t5593 - 0.10001700163888888889e0 * t26604 * t30072 - 0.10001700163888888889e0 * t5813 * t5579 * t72 * t16887 - 0.22226000364197530865e-1 * t104682 + 0.10001700163888888889e0 * t5829 * t5579 * t72 * t16825 - 0.18122740165211489339e1 * t105212 * t118744 + 0.26671200437037037037e0 * t5813 * t22767 * t30071;
    (t118744, t118750)
}
