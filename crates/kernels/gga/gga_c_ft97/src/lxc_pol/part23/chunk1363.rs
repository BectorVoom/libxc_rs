//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1363/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1363<F: Float>(t218: F, t5260: F, t25057: F, t820: F, t5295: F, t1196: F, t6793: F, t112223: F, t1208: F, t123230: F, t6256: F, t123056: F, t123061: F, t123591: F, t123676: F, t14721: F, t14729: F, t14766: F, t231: F, t25049: F, t25077: F, t25112: F, t27642: F, t28548: F, t28552: F, t28558: F, t5284: F, t6045: F, t70779: F, t7607: F, t811: F, t82940: F, t82944: F, t82960: F, t98539: F, t98563: F) -> (F, F, F, F, F, F) {
    let t127255 = t218 * t5260;
    let t127257 = t25057 * t127255 * t820;
    let t127260 = t218 * t5295;
    let t127262 = t25057 * t127260 * t820;
    let t127276 = t6793 * t1196;
    let t127278 = t112223 * t127276 * t820;
    let t127281 = t6793 * t1208;
    let t127283 = t112223 * t127281 * t820;
    let t127295 = t6256 * t123230;
    let t127297 = -0.16299066933744855968e0 * t6256 * t123591 + 0.20003400327777777778e0 * t25049 * t6045 * t231 * t82940 - 0.30005100491666666667e0 * t25112 * t6045 * t231 * t82944 - 0.45306850413028723348e0 * t14721 * t127257 - 0.45306850413028723348e0 * t14729 * t127262 - 0.17780800291358024692e0 * t25077 * t27642 * t28548 + 0.46992870109762241323e0 * t28558 * t123676 - 0.10001700163888888889e0 * t28552 * t123056 + 0.13335600218518518519e0 * t28552 * t123061 + 0.45306850413028723348e0 * t14766 * t127257 - 0.43791161479435967988e1 * t70779 * t127278 - 0.21895580739717983994e1 * t7607 * t127283 - 0.60010200983333333334e0 * t98563 * t6045 * t231 * t5284 * t811 + 0.12002040196666666667e1 * t98539 * t6045 * t231 * t82960 - 0.74086667880658436217e-2 * t127295;
    (t127260, t127262, t127278, t127281, t127283, t127297)
}
