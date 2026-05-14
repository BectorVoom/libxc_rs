//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1427/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1427<F: Float>(t113: F, t32319: F, t2147: F, t2148: F, t6363: F, t22820: F, t3052: F, t7338: F, t1568: F, t7623: F, t1569: F, t6535: F, t22863: F, t27074: F, t27078: F, t27080: F, t30969: F, t30988: F, t31018: F, t31020: F, t31024: F, t31030: F) -> (F, F) {
    let t34573 = t32319 * t113;
    let t34575 = t2147 * t2148 * t34573;
    let t34578 = t32319 * t6363;
    let t34580 = t22820 * t2148 * t34578;
    let t34582 = t7338 * t3052;
    let t34584 = t7623 * t1568 * t34582;
    let t34586 = t32319 * t1569;
    let t34588 = t6535 * t2148 * t34586;
    let t34596 = 0.52396431978519890151e-1 * t30969 - 0.58218257753910989057e-2 * t34575 - 0.59329162131926993721e1 * t27074 - 0.34930954652346593435e-1 * t34580 - 0.16463622957338778996e-1 * t34584 + 0.34930954652346593433e-1 * t34588 - t27078 + t27080 + 0.20803732176130244552e1 * t30988 - 0.349099539297917348e0 * t22863 - 0.11524536070137145298e1 * t31018 + 0.34672886960217074253e0 * t31020 + 0.34672886960217074253e0 * t31024 + 0.69345773920434148506e0 * t31030;
    (t34582, t34596)
}
