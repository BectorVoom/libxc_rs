//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1370/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1370<F: Float>(t33289: F, t33292: F, t33295: F, t33298: F, t33301: F, t33305: F, t33313: F, t33315: F, t33320: F, t33324: F, t33326: F, t33330: F, t33333: F, t33336: F, t33339: F, t33341: F, t33343: F, t33346: F, t33349: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36535 = F::new(0.63350674672043801542e-5) * t33289;
    let t36536 = F::new(0.2318836277704281739e-4) * t33292;
    let t36537 = F::new(0.43440462632258606772e-4) * t33295;
    let t36538 = F::new(0.43440462632258606772e-4) * t33298;
    let t36539 = F::new(0.21720231316129303386e-4) * t33301;
    let t36540 = F::new(0.17632363114482012216e-5) * t33305;
    let t36542 = F::new(0.1371666545474996961e-6) * t33313;
    let t36543 = F::new(0.3243554543208642639e-2) * t33315;
    let t36556 = F::new(0.43440462632258606772e-4) * t33320 - F::new(0.69504740211613770836e-3) * t33324 - F::new(0.3243554543208642639e-2) * t33326 + F::new(0.1433927048577202691e-8) * t33330 - F::new(0.2318836277704281739e-4) * t33333 - F::new(0.12290803273518880209e-8) * t33336 + F::new(0.16387737698025173612e-8) * t33339 + F::new(0.3243554543208642639e-2) * t33341 - F::new(0.61320337121513228211e-3) * t33343 + F::new(0.22466860691349365008e-6) * t33346 + F::new(0.11594181388521408695e-4) * t33349;
    (t36535, t36536, t36537, t36538, t36539, t36540, t36542, t36543, t36556)
}
