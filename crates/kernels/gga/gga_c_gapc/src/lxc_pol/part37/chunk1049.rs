//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1049/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1049<F: Float>(t11493: F, t11497: F, t11501: F, t11504: F, t11506: F, t11510: F, t11524: F, t11527: F, t11529: F, t11547: F, t11552: F, t11564: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12115 = F::new(0.4637672555408563478e-4) * t11493;
    let t12116 = F::new(0.38647271295071362317e-6) * t11497;
    let t12117 = F::new(0.68714848362636882201e-6) * t11501;
    let t12118 = F::new(0.22510123728325872388e-7) * t11504;
    let t12119 = F::new(0.22510123728325872388e-6) * t11506;
    let t12120 = F::new(0.30353495895471971565e-6) * t11510;
    let t12123 = F::new(0.25301920572916666668e-5) * t11524;
    let t12124 = F::new(0.25301920572916666668e-5) * t11527;
    let t12125 = F::new(0.16217772716043213195e-2) * t11529;
    let t12129 = F::new(0.1422820120100248667e-7) * t11547;
    let t12131 = F::new(0.11594181388521408695e-4) * t11552;
    let t12135 = F::new(0.11594181388521408695e-4) * t11564;
    (t12115, t12116, t12117, t12118, t12119, t12120, t12123, t12124, t12125, t12129, t12131, t12135)
}
