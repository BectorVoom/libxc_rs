//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1256/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1256(t1890: f64, t3487: f64, t7805: f64, t7810: f64, t107: f64, t10809: f64, t787: f64, t20019: f64, t7292: f64, t11061: f64, t14555: f64, t5241: f64) -> (f64, f64, f64, f64) {
    let t33289 = t1890 * t3487;
    let t33291 = t7810 * t33289 * t7805;
    let t33292 = 0.19171462976960374838e1_f64 * t33291;
    let t33294 = t787 * t10809 * t107;
    let t33297 = 0.95334639871601137784e0_f64 * t33294 * t20019 * t7292;
    let t33299 = 0.15337170381568299871e1_f64 * t14555 * t11061;
    let t33308 = t5241 * t3487;
    (t33292, t33297, t33299, t33308)
}
