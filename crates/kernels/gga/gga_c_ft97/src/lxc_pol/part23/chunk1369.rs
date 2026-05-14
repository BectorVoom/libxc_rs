//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1369/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1369<F: Float>(t127429: F, t285: F, t112282: F, t1208: F, t25057: F, t28637: F, t4125: F, t127359: F, t7003: F, t28662: F, t30727: F, t1091: F, t111915: F, t112196: F, t112205: F, t112219: F, t112295: F, t123226: F, t127362: F, t127426: F, t14721: F, t14766: F, t19072: F, t25070: F, t27642: F, t28599: F, t28680: F, t35877: F, t4126: F, t4635: F, t5265: F, t6035: F, t6979: F, t704: F, t811: F, t98530: F) -> (F, F, F, F) {
    let t127430 = t285 * t127429;
    let t127434 = t25057 * t112282 * t1208;
    let t127438 = t25057 * t28637 * t4125;
    let t127456 = t7003 * t127359;
    let t127466 = t30727 * t28662;
    let t127469 = 0.45306850413028723348e0 * t14766 * t127426 - 0.94171484795751865043e-2 * t127430 * t123226 - 0.90613700826057446696e0 * t14721 * t127434 - 0.90613700826057446696e0 * t14721 * t127438 - 0.12002040196666666667e1 * t111915 * t6035 * t35877 * t19072 + 0.22226000364197530866e-1 * t112196 + 0.54738951849294959988e0 * t5265 * t6979 * t4126 - 0.22226000364197530866e-1 * t98530 - 0.33339000546296296298e-1 * t25070 * t6035 * t704 * t4635 * t811 - 0.59269334304526748974e-1 * t112205 + 0.14500657062440920179e1 * t127456 * t127362 - 0.66678001092592592595e-1 * t25070 * t6035 * t112295 * t1091 - t112219 + 0.17780800291358024692e0 * t25070 * t27642 * t28599 + 0.28195722065857344794e1 * t28680 * t127466;
    (t127434, t127438, t127466, t127469)
}
