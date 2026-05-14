//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1036/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1036<F: Float>(t281: F, t30760: F, t287: F, t30763: F, t290: F, t1471: F, t5231: F, t1208: F, t28637: F, t25057: F, t5009: F, t30674: F, t14: F, t816: F, t231: F, t1408: F, t1420: F, t1472: F, t14721: F, t14766: F, t19101: F, t19132: F, t19135: F, t22082: F, t291: F, t31473: F, t31477: F, t31502: F, t31508: F, t31515: F, t4094: F, t4104: F, t4113: F, t5265: F, t7003: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31519 = t281 * t30760;
    let t31520 = t30763 * t287;
    let t31521 = t31520 * t290;
    let t31526 = t5231 * t1471;
    let t31529 = t28637 * t1208;
    let t31530 = t25057 * t31529;
    let t31535 = t281 * t5009;
    let t31536 = t30674 * t287;
    let t31537 = t31535 * t31536;
    let t31538 = t816 * t14;
    let t31539 = t31538 * t231;
    let t31542 = t290 * t30760;
    let t31543 = t31542 * t30763;
    let t31548 = 0.22653425206514361674e0 * t1472 * t31473 - 0.21895580739717983994e1 * t19132 * t31502 + 0.10947790369858991997e1 * t19135 * t31477 - 0.45306850413028723348e0 * t4104 * t31508 + 0.45306850413028723348e0 * t4094 * t31508 + 0.43791161479435967988e1 * t19101 * t31502 + 0.46528109071900715989e1 * t5265 * t31515 * t291 - 0.61104346057464762978e-1 * t31519 * t31521 + 0.45306850413028723348e0 * t22082 * t1408 - 0.10001700163888888889e0 * t31526 * t1420 + 0.90613700826057446696e0 * t14766 * t31530 - 0.90613700826057446696e0 * t14721 * t31530 - 0.14125722719362779757e-1 * t31537 * t31539 - 0.40736230704976508653e-1 * t4113 * t31543 + 0.12220869211492952596e0 * t7003 * t31543;
    (t31519, t31526, t31530, t31535, t31536, t31537, t31538, t31542, t31548)
}
