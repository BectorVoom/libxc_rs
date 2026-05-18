//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1122/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1122<F: Float>(t2725: F, t6789: F, t285: F, t150688: F, t6250: F, t1701: F, t28638: F, t33898: F, t35367: F, t150696: F, t33948: F, t1196: F, t142697: F, t142704: F, t14766: F, t150336: F, t150372: F, t153039: F, t153063: F, t153071: F, t153075: F, t153077: F, t153081: F, t153083: F, t153087: F, t153091: F, t153094: F, t153104: F, t153112: F, t153113: F, t19101: F, t19107: F, t19132: F, t2344: F, t28547: F, t28660: F, t28677: F, t33426: F, t684: F, t706: F) -> (F, F, F, F, F) {
    let t153116 = t2725 * t6789;
    let t153117 = t285 * t153116;
    let t153118 = t150688 * t6250;
    let t153121 = t1701 * t28638;
    let t153124 = t35367 * t33898;
    let t153127 = t33948 * t150696;
    let t153129 = F::new(0.4445955829703778972e-1) * t153063 * t150336 + F::new(0.3531430679840694939e-2) * t153071 * t706 - F::new(0.58778941170896004276e-1) * t153075 * t153077 + F::new(0.88168411756344006414e-1) * t153081 * t153083 - F::new(0.82108427773942439976e0) * t19101 * t153087 + F::new(0.41054213886971219988e0) * t19107 * t153091 + F::new(0.45306850413028723348e0) * t14766 * t153094 + F::new(0.14500657062440920179e1) * t28677 * t153039 + F::new(0.53351469956445347664e-1) * t142697 * t33426 * t2344 * t1196 * t684 - F::new(0.14227058655052092711e0) * t153104 - F::new(0.80027204934668021496e-1) * t142704 * t33426 * t150372 * t28547 + F::new(0.41054213886971219988e0) * t19132 * t153087 + F::new(0.42377168158088339266e-1) * t153112 * t153113 - F::new(0.42377168158088339266e-1) * t153117 * t153118 + F::new(0.45306850413028723348e0) * t14766 * t153121 + F::new(0.14500657062440920179e1) * t28660 * t153124 + F::new(0.17783823318815115888e-1) * t153127;
    (t153116, t153118, t153121, t153124, t153129)
}
