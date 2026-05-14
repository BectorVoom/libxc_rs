//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1192/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1192<F: Float>(t1196: F, t2440: F, t109124: F, t6256: F, t14724: F, t25057: F, t109168: F, t1472: F, t2726: F, t28637: F, t28628: F, t108830: F, t112086: F, t14721: F, t14742: F, t231: F, t2405: F, t25070: F, t25112: F, t28552: F, t54840: F, t55011: F, t6035: F, t6045: F, t70440: F, t98530: F, t98535: F) -> (F, F, F, F) {
    let t112185 = t2440 * t1196;
    let t112196 = t6256 * t109124;
    let t112202 = t25057 * t14724;
    let t112205 = t1472 * t109168;
    let t112208 = t25057 * t28637 * t2726;
    let t112212 = t25057 * t28628 * t2726;
    let t112215 = -0.44452000728395061729e-1 * t25070 * t6035 * t112185 * t2405 + 0.33339000546296296298e-1 * t28552 * t108830 - 0.30005100491666666667e0 * t25112 * t6045 * t231 * t70440 - 0.77791001274691358028e-1 * t112196 - 0.44452000728395061731e-1 * t98530 - 0.10001700163888888889e0 * t98535 - 0.24163653553615319118e1 * t14742 * t112086 + 0.48327307107230638236e1 * t14721 * t112202 - 0.29634667152263374487e-1 * t112205 - 0.90613700826057446696e0 * t54840 * t112208 + 0.13592055123908617004e1 * t55011 * t112212;
    (t112202, t112208, t112212, t112215)
}
