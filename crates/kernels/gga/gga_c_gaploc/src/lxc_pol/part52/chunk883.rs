//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 883/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk883<F: Float>(t11622: F, t1445: F, t2530: F, t813: F, t43925: F, t43927: F, t43930: F, t2365: F, t35500: F, t7390: F, t36798: F, t787: F, t9824: F) -> (F, F, F, F, F, F) {
    let t45792 = F::new(0.46011511144704899612e1) * t813 * t1445 * t11622 * t2530;
    let t45793 = F::new(0.17875244975925213335e0) * t43925;
    let t45794 = F::new(0.17875244975925213335e0) * t43927;
    let t45795 = F::new(0.19171462976960374838e0) * t43930;
    let t45797 = t7390 * t2365 * t35500;
    let t45798 = F::new(0.14896037479937677779e-1) * t45797;
    let t45800 = t787 * t36798 * t9824;
    (t45792, t45793, t45794, t45795, t45798, t45800)
}
