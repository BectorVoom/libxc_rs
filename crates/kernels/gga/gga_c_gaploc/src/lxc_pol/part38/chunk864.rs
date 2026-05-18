//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 864/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk864<F: Float>(t13525: F, t325: F, t550: F, t42973: F, t2581: F, t1841: F, t35440: F, t11657: F, t2554: F, t7064: F, t35385: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t44771 = t325 * t13525;
    let t44772 = t550 * t44771;
    let t44776 = F::new(0.1281754371690370714e-2) * t42973;
    let t44777 = t550 * t2581;
    let t44780 = F::new(0.10254034973522965711e-1) * t1841 * t35440 * t44777;
    let t44785 = t7064 * t11657 * t2554;
    let t44786 = F::new(0.32043859292259267849e-3) * t44785;
    let t44787 = t883 * t35385;
    (t44771, t44772, t44776, t44777, t44780, t44786, t44787)
}
