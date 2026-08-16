//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 743/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk743<F: Float>(t15625: F, t359: F, t356: F, t89: F, t1597: F, t4441: F, t63: F, t7857: F, t3099: F, t930: F, t374: F, t11375: F, t938: F) -> (F, F, F, F, F) {
    let t15626 = t359 * t15625;
    let t15628 = t89 * t356 * t15626;
    let t15630 = t4441 * t1597;
    let t15631 = t15630 * t63;
    let t15632 = t7857 * t15631;
    let t15635 = t930 * t3099;
    let t15636 = t374 * t15635;
    let t15639 = t11375 * t938;
    (t15628, t15630, t15632, t15636, t15639)
}
