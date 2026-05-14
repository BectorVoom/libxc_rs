//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 995/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk995<F: Float>(t26791: F, t6533: F, t5329: F, t356: F, t6497: F, t303: F, t26772: F, t6487: F, t6276: F, t7704: F, t4947: F, t26695: F, t6272: F, t4939: F, t2173: F, t26685: F, t26856: F, t27832: F, t27967: F, t27969: F, t27975: F, t27981: F, t28988: F, t7703: F, t8038: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28996 = t26791 * t6533;
    let t28997 = t5329 * t28996;
    let t29000 = t356 * t6497;
    let t29001 = t303 * t29000;
    let t29003 = t26772 * t6487;
    let t29004 = t303 * t29003;
    let t29006 = t7704 * t6276;
    let t29007 = t4947 * t29006;
    let t29010 = t26695 * t6272;
    let t29011 = t4939 * t29010;
    let t29022 = t26856 - 0.13901041666666666667e-2 * t2173 * t28997 - 0.55273148148148148147e-3 * t29001 + 0.49745833333333333332e-2 * t29004 - 0.23168402777777777778e-3 * t7703 * t29007 - 0.30891203703703703704e-3 * t7703 * t29011 - 0.18550940104166666667e-3 * t26685 * t28988 - 0.46336805555555555556e-3 * t27832 * t8038 - 0.46336805555555555556e-3 * t27967 - 0.33163888888888888888e-2 * t27969 + 0.46336805555555555556e-3 * t27975 + 0.61836467013888888889e-4 * t27981;
    (t28996, t28997, t29000, t29001, t29003, t29004, t29006, t29007, t29010, t29011, t29022)
}
