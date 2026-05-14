//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1085/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1085<F: Float>(t108530: F, t505: F, t4075: F, t27521: F, t27523: F, t27574: F, t1113: F, t2393: F, t2395: F, t108503: F, t108504: F, t108508: F, t108509: F, t108518: F, t108519: F, t108525: F, t108526: F, t1096: F, t213: F, t231: F, t232: F, t24276: F, t24278: F, t24324: F, t2455: F, t27537: F, t27609: F, t27616: F, t27618: F, t27646: F, t3762: F, t6045: F, t66612: F, t66619: F, t6819: F, t96421: F, t96424: F, t96600: F, t96602: F, t96716: F, t96717: F) -> (F, F) {
    let t108531 = t108530 * t505;
    let t108532 = t4075 * t108531;
    let t108550 = 0.60548059007656442388e-3 * t27521 * t27574 * t27523;
    let t108561 = t1113 * t2393 * t2395;
    let t108570 = 0.1979569051908603763e-3 * t24276 * t108503 * t108504 - 0.29693535778629056444e-3 * t96716 * t108508 * t108509 - 0.29693535778629056444e-3 * t96716 * t27537 * t27646 * t3762 + 0.29693535778629056444e-3 * t108518 * t108519 * t108509 + 0.3520097786805302698e-5 * t108525 * t108526 * t108509 - 0.98910212891072794759e-5 * t96600 * t96602 * t108532 - 0.29693535778629056444e-3 * t96716 * t24278 * t1096 * t96717 - 0.29693535778629056444e-4 * t24276 * t96424 * t108532 + 0.90822088511484663582e-3 * t27521 * t6819 * t231 * t213 * t2455 + t108550 - 0.22983699016666666666e0 * t24324 * t6045 * t231 * t66612 - 0.11491849508333333333e0 * t24324 * t6045 * t231 * t66619 + 0.49489226297715094073e-4 * t96421 - 0.89080607335887169332e-4 * t27609 * t232 * t108561 - 0.10560293360415908094e-5 * t27616 * t27618 * t213 * t2393 * t2395;
    (t108561, t108570)
}
