//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 909/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk909<F: Float>(t35481: F, t6051: F, t109246: F, t123181: F, t13411: F, t150731: F, t150764: F, t150770: F, t150773: F, t150776: F, t150787: F, t150789: F, t150791: F, t27616: F, t27665: F, t27672: F, t33368: F, t33385: F, t33404: F, t33418: F, t36835: F, t3774: F, t66563: F) -> (F,) {
    let t150793 = t35481 * t6051;
    let t150797 = 0.25845121844514357744e-4 * t33418 * t150764 - 0.60102574844279699039e-6 * t13411 * t66563 * t150770 - 0.25845121844514357744e-4 * t150773 * t33385 + 0.89080607335887169333e-3 * t150776 * t33368 - 0.20715606998445758511e-4 * t123181 * t36835 * t109246 * t27672 - 0.39601100101559655353e-5 * t27616 * t33404 * t27665 - 0.20869152414369355073e-1 * t150787 + 0.6809984893827160494e-1 * t150789 - 0.25537443351851851852e-1 * t150791 - 0.25537443351851851852e-1 * t150793 - 0.51690243689028715488e-5 * t3774 * t150731;
    (t150797,)
}
